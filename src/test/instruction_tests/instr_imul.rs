use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn imul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: Some(Literal16(28909)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 215, 237, 112], OperandSize::Word)
}

#[test]
fn imul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 19142, Some(OperandSize::Word), None)), operand3: Some(Literal16(22136)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 152, 198, 74, 120, 86], OperandSize::Word)
}

#[test]
fn imul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: Some(Literal16(15395)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 201, 35, 60], OperandSize::Dword)
}

#[test]
fn imul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand3: Some(Literal16(5953)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 25, 65, 23], OperandSize::Dword)
}

#[test]
fn imul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: Some(Literal16(15003)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 245, 155, 58], OperandSize::Qword)
}

#[test]
fn imul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1332533581, Some(OperandSize::Word), None)), operand3: Some(Literal16(546)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 36, 133, 77, 217, 108, 79, 34, 2], OperandSize::Qword)
}

#[test]
fn imul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: Some(Literal32(180712343)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 203, 151, 115, 197, 10], OperandSize::Word)
}

#[test]
fn imul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(BX, 27563, Some(OperandSize::Dword), None)), operand3: Some(Literal32(705861353)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 151, 171, 107, 233, 150, 18, 42], OperandSize::Word)
}

#[test]
fn imul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: Some(Literal32(130439210)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 212, 42, 88, 198, 7], OperandSize::Dword)
}

#[test]
fn imul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: Some(Literal32(773180833)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 50, 161, 205, 21, 46], OperandSize::Dword)
}

#[test]
fn imul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: Some(Literal32(1907445616)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 225, 112, 79, 177, 113], OperandSize::Qword)
}

#[test]
fn imul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RSI, 1195705535, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1236041984)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 150, 191, 4, 69, 71, 0, 129, 172, 73], OperandSize::Qword)
}

#[test]
fn imul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: Some(Literal32(1116174468)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 230, 132, 120, 135, 66], OperandSize::Qword)
}

#[test]
fn imul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1159063093, Some(OperandSize::Qword), None)), operand3: Some(Literal32(507300892)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 20, 213, 53, 230, 21, 69, 28, 204, 60, 30], OperandSize::Qword)
}

#[test]
fn imul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 247, 34], OperandSize::Word)
}

#[test]
fn imul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 26, 22], OperandSize::Word)
}

#[test]
fn imul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 209, 111], OperandSize::Dword)
}

#[test]
fn imul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Word), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 44, 187, 65], OperandSize::Dword)
}

#[test]
fn imul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 250, 24], OperandSize::Qword)
}

#[test]
fn imul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 63, 32], OperandSize::Qword)
}

#[test]
fn imul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 201, 43], OperandSize::Word)
}

#[test]
fn imul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 31579, Some(OperandSize::Dword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 138, 91, 123, 68], OperandSize::Word)
}

#[test]
fn imul_23() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 201, 119], OperandSize::Dword)
}

#[test]
fn imul_24() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 935477948, Some(OperandSize::Dword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 60, 141, 188, 66, 194, 55, 41], OperandSize::Dword)
}

#[test]
fn imul_25() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 245, 113], OperandSize::Qword)
}

#[test]
fn imul_26() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RAX, 1056562118, Some(OperandSize::Dword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 144, 198, 219, 249, 62, 49], OperandSize::Qword)
}

#[test]
fn imul_27() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 215, 59], OperandSize::Qword)
}

#[test]
fn imul_28() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 44, 193, 62], OperandSize::Qword)
}

#[test]
fn imul_29() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 241], OperandSize::Word)
}

#[test]
fn imul_30() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 53], OperandSize::Word)
}

#[test]
fn imul_31() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 205], OperandSize::Dword)
}

#[test]
fn imul_32() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(EBX, 1191254665, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 155, 137, 26, 1, 71], OperandSize::Dword)
}

#[test]
fn imul_33() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 228], OperandSize::Qword)
}

#[test]
fn imul_34() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 60, 81], OperandSize::Qword)
}

#[test]
fn imul_35() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 201], OperandSize::Word)
}

#[test]
fn imul_36() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 110, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 90, 110], OperandSize::Word)
}

#[test]
fn imul_37() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 231], OperandSize::Dword)
}

#[test]
fn imul_38() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(EBX, 1460760013, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 187, 205, 109, 17, 87], OperandSize::Dword)
}

#[test]
fn imul_39() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 218], OperandSize::Qword)
}

#[test]
fn imul_40() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RAX, 1258054619, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 152, 219, 99, 252, 74], OperandSize::Qword)
}

#[test]
fn imul_41() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 219], OperandSize::Qword)
}

#[test]
fn imul_42() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 422434039, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 60, 141, 247, 212, 45, 25], OperandSize::Qword)
}

#[test]
fn imul_43() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 234], OperandSize::Word)
}

#[test]
fn imul_44() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(BP, 170, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 174, 170, 0], OperandSize::Word)
}

#[test]
fn imul_45() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 235], OperandSize::Dword)
}

#[test]
fn imul_46() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 46], OperandSize::Dword)
}

#[test]
fn imul_47() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 233], OperandSize::Qword)
}

#[test]
fn imul_48() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 43], OperandSize::Qword)
}

#[test]
fn imul_49() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 234], OperandSize::Word)
}

#[test]
fn imul_50() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 44], OperandSize::Word)
}

#[test]
fn imul_51() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 236], OperandSize::Dword)
}

#[test]
fn imul_52() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 44, 208], OperandSize::Dword)
}

#[test]
fn imul_53() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 238], OperandSize::Qword)
}

#[test]
fn imul_54() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 40], OperandSize::Qword)
}

#[test]
fn imul_55() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 238], OperandSize::Word)
}

#[test]
fn imul_56() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 41], OperandSize::Word)
}

#[test]
fn imul_57() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 235], OperandSize::Dword)
}

#[test]
fn imul_58() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1564256524, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 44, 69, 12, 169, 60, 93], OperandSize::Dword)
}

#[test]
fn imul_59() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 233], OperandSize::Qword)
}

#[test]
fn imul_60() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(RBX, Four, 792833437, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 44, 157, 157, 173, 65, 47], OperandSize::Qword)
}

#[test]
fn imul_61() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 238], OperandSize::Qword)
}

#[test]
fn imul_62() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(RDI, Four, 30854395, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 44, 189, 251, 204, 214, 1], OperandSize::Qword)
}

