use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn imul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: Some(Literal16(23957)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 220, 149, 93], OperandSize::Word)
}

#[test]
fn imul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(BP, 5084, Some(OperandSize::Word), None)), operand3: Some(Literal16(14985)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 150, 220, 19, 137, 58], OperandSize::Word)
}

#[test]
fn imul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: Some(Literal16(26145)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 214, 33, 102], OperandSize::Dword)
}

#[test]
fn imul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1418682501, Some(OperandSize::Word), None)), operand3: Some(Literal16(22570)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 44, 125, 133, 96, 143, 84, 42, 88], OperandSize::Dword)
}

#[test]
fn imul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: Some(Literal16(31167)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 251, 191, 121], OperandSize::Qword)
}

#[test]
fn imul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Word), None)), operand3: Some(Literal16(16355)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 52, 246, 227, 63], OperandSize::Qword)
}

#[test]
fn imul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: Some(Literal32(415111802)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 239, 122, 26, 190, 24], OperandSize::Word)
}

#[test]
fn imul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(BP, 30702, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1732298620)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 142, 238, 119, 124, 199, 64, 103], OperandSize::Word)
}

#[test]
fn imul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: Some(Literal32(632394199)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 246, 215, 145, 177, 37], OperandSize::Dword)
}

#[test]
fn imul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 256590511, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1978134375)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 44, 77, 175, 66, 75, 15, 103, 239, 231, 117], OperandSize::Dword)
}

#[test]
fn imul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: Some(Literal32(953845103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 222, 111, 133, 218, 56], OperandSize::Qword)
}

#[test]
fn imul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1812929937)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 49, 145, 29, 15, 108], OperandSize::Qword)
}

#[test]
fn imul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBP)), operand3: Some(Literal32(1921696276)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 245, 20, 194, 138, 114], OperandSize::Qword)
}

#[test]
fn imul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal32(1566321195)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 44, 122, 43, 42, 92, 93], OperandSize::Qword)
}

#[test]
fn imul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 214, 106], OperandSize::Word)
}

#[test]
fn imul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 57, Some(OperandSize::Word), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 90, 57, 24], OperandSize::Word)
}

#[test]
fn imul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 205, 56], OperandSize::Dword)
}

#[test]
fn imul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 22, 44], OperandSize::Dword)
}

#[test]
fn imul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 205, 33], OperandSize::Qword)
}

#[test]
fn imul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1699102146, Some(OperandSize::Word), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 180, 152, 194, 61, 70, 101, 47], OperandSize::Qword)
}

#[test]
fn imul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 205, 16], OperandSize::Word)
}

#[test]
fn imul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 10610, Some(OperandSize::Dword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 185, 114, 41, 112], OperandSize::Word)
}

#[test]
fn imul_23() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 250, 36], OperandSize::Dword)
}

#[test]
fn imul_24() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EAX, 363229199, Some(OperandSize::Dword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 152, 15, 112, 166, 21, 68], OperandSize::Dword)
}

#[test]
fn imul_25() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 245, 15], OperandSize::Qword)
}

#[test]
fn imul_26() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 580831072, Some(OperandSize::Dword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 188, 113, 96, 199, 158, 34, 9], OperandSize::Qword)
}

#[test]
fn imul_27() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RDX)), operand2: Some(Direct(RCX)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 209, 28], OperandSize::Qword)
}

#[test]
fn imul_28() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 564286801, Some(OperandSize::Qword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 36, 85, 81, 85, 162, 33, 116], OperandSize::Qword)
}

#[test]
fn imul_29() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 250], OperandSize::Word)
}

#[test]
fn imul_30() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(SI, 23698, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 188, 146, 92], OperandSize::Word)
}

#[test]
fn imul_31() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 227], OperandSize::Dword)
}

#[test]
fn imul_32() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 20, 128], OperandSize::Dword)
}

#[test]
fn imul_33() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 249], OperandSize::Qword)
}

#[test]
fn imul_34() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 828348499, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 140, 152, 83, 152, 95, 49], OperandSize::Qword)
}

#[test]
fn imul_35() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 217], OperandSize::Word)
}

#[test]
fn imul_36() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 9444, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 139, 228, 36], OperandSize::Word)
}

#[test]
fn imul_37() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 218], OperandSize::Dword)
}

#[test]
fn imul_38() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 60, 190], OperandSize::Dword)
}

#[test]
fn imul_39() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 228], OperandSize::Qword)
}

#[test]
fn imul_40() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1806484333, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 172, 178, 109, 195, 172, 107], OperandSize::Qword)
}

#[test]
fn imul_41() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 205], OperandSize::Qword)
}

#[test]
fn imul_42() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 915416682, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 148, 187, 106, 38, 144, 54], OperandSize::Qword)
}

#[test]
fn imul_43() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 234], OperandSize::Word)
}

#[test]
fn imul_44() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(SI, 136, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 172, 136, 0], OperandSize::Word)
}

#[test]
fn imul_45() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 233], OperandSize::Dword)
}

#[test]
fn imul_46() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 41], OperandSize::Dword)
}

#[test]
fn imul_47() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 235], OperandSize::Qword)
}

#[test]
fn imul_48() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 44, 119], OperandSize::Qword)
}

#[test]
fn imul_49() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 239], OperandSize::Word)
}

#[test]
fn imul_50() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Memory(25075, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 46, 243, 97], OperandSize::Word)
}

#[test]
fn imul_51() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 236], OperandSize::Dword)
}

#[test]
fn imul_52() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(EAX, 699120965, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 168, 69, 189, 171, 41], OperandSize::Dword)
}

#[test]
fn imul_53() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 238], OperandSize::Qword)
}

#[test]
fn imul_54() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 47], OperandSize::Qword)
}

#[test]
fn imul_55() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 236], OperandSize::Word)
}

#[test]
fn imul_56() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(BX, 32327, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 175, 71, 126], OperandSize::Word)
}

#[test]
fn imul_57() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 237], OperandSize::Dword)
}

#[test]
fn imul_58() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 483330470, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 172, 249, 166, 9, 207, 28], OperandSize::Dword)
}

#[test]
fn imul_59() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 237], OperandSize::Qword)
}

#[test]
fn imul_60() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 363346933, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 172, 203, 245, 59, 168, 21], OperandSize::Qword)
}

#[test]
fn imul_61() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 235], OperandSize::Qword)
}

#[test]
fn imul_62() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 44, 95], OperandSize::Qword)
}

