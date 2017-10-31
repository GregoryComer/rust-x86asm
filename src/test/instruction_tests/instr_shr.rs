use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 235, 76], OperandSize::Word)
}

#[test]
fn shr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(BP, 11044, Some(OperandSize::Byte), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 174, 36, 43, 110], OperandSize::Word)
}

#[test]
fn shr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 233, 64], OperandSize::Dword)
}

#[test]
fn shr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1050512184, Some(OperandSize::Byte), None)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 172, 95, 56, 139, 157, 62, 47], OperandSize::Dword)
}

#[test]
fn shr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 234, 87], OperandSize::Qword)
}

#[test]
fn shr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 44, 129, 76], OperandSize::Qword)
}

#[test]
fn shr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 235, 90], OperandSize::Qword)
}

#[test]
fn shr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(RDI, Four, 489995851, Some(OperandSize::Byte), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 44, 189, 75, 190, 52, 29, 110], OperandSize::Qword)
}

#[test]
fn shr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BP)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 237, 108], OperandSize::Word)
}

#[test]
fn shr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 42, 21], OperandSize::Word)
}

#[test]
fn shr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(SP)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 236, 24], OperandSize::Dword)
}

#[test]
fn shr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1516627642, Some(OperandSize::Word), None)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 172, 211, 186, 230, 101, 90, 15], OperandSize::Dword)
}

#[test]
fn shr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(SP)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 236, 64], OperandSize::Qword)
}

#[test]
fn shr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1345167177, Some(OperandSize::Word), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 44, 221, 73, 159, 45, 80, 113], OperandSize::Qword)
}

#[test]
fn shr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 238, 23], OperandSize::Word)
}

#[test]
fn shr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(SI, 181, Some(OperandSize::Dword), None)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 172, 181, 0, 32], OperandSize::Word)
}

#[test]
fn shr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 234], OperandSize::Dword)
}

#[test]
fn shr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 44, 217, 38], OperandSize::Dword)
}

#[test]
fn shr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 235, 37], OperandSize::Qword)
}

#[test]
fn shr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 44, 94, 65], OperandSize::Qword)
}

#[test]
fn shr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(RSI)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 238, 43], OperandSize::Qword)
}

#[test]
fn shr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 101235201, Some(OperandSize::Qword), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 172, 241, 1, 186, 8, 6, 65], OperandSize::Qword)
}

#[test]
fn shr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 235], OperandSize::Word)
}

#[test]
fn shr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 47], OperandSize::Word)
}

#[test]
fn shr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 235], OperandSize::Dword)
}

#[test]
fn shr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1901718551, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 172, 177, 23, 236, 89, 113], OperandSize::Dword)
}

#[test]
fn shr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 235], OperandSize::Qword)
}

#[test]
fn shr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1530406710, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 172, 134, 54, 39, 56, 91], OperandSize::Qword)
}

#[test]
fn shr_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 234], OperandSize::Qword)
}

#[test]
fn shr_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RDX, 1586821131, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 170, 11, 248, 148, 94], OperandSize::Qword)
}

#[test]
fn shr_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 235], OperandSize::Word)
}

#[test]
fn shr_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 239, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 171, 239, 0], OperandSize::Word)
}

#[test]
fn shr_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 238], OperandSize::Dword)
}

#[test]
fn shr_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 44, 203], OperandSize::Dword)
}

#[test]
fn shr_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 239], OperandSize::Qword)
}

#[test]
fn shr_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RDX, 580585561, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 170, 89, 8, 155, 34], OperandSize::Qword)
}

#[test]
fn shr_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 238], OperandSize::Word)
}

#[test]
fn shr_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 8469, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 170, 21, 33], OperandSize::Word)
}

#[test]
fn shr_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 239], OperandSize::Dword)
}

#[test]
fn shr_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 48114840, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 44, 253, 152, 44, 222, 2], OperandSize::Dword)
}

#[test]
fn shr_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 236], OperandSize::Qword)
}

#[test]
fn shr_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1798561677, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 44, 157, 141, 223, 51, 107], OperandSize::Qword)
}

#[test]
fn shr_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(RDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 239], OperandSize::Qword)
}

#[test]
fn shr_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 42], OperandSize::Qword)
}

#[test]
fn shr_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 235], OperandSize::Word)
}

#[test]
fn shr_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 16963, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 171, 67, 66], OperandSize::Word)
}

#[test]
fn shr_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 235], OperandSize::Dword)
}

#[test]
fn shr_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 711495669, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 44, 245, 245, 143, 104, 42], OperandSize::Dword)
}

#[test]
fn shr_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 233], OperandSize::Qword)
}

#[test]
fn shr_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RBX, 866682922, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 171, 42, 136, 168, 51], OperandSize::Qword)
}

#[test]
fn shr_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 234], OperandSize::Qword)
}

#[test]
fn shr_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RSI, 1771503786, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 174, 170, 0, 151, 105], OperandSize::Qword)
}

#[test]
fn shr_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 234], OperandSize::Word)
}

#[test]
fn shr_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 21555, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 169, 51, 84], OperandSize::Word)
}

#[test]
fn shr_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 233], OperandSize::Dword)
}

#[test]
fn shr_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 666027490, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 44, 213, 226, 197, 178, 39], OperandSize::Dword)
}

#[test]
fn shr_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 235], OperandSize::Qword)
}

#[test]
fn shr_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RAX, 1167190420, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 168, 148, 233, 145, 69], OperandSize::Qword)
}

#[test]
fn shr_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 236], OperandSize::Word)
}

#[test]
fn shr_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 29, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 107, 29], OperandSize::Word)
}

#[test]
fn shr_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 234], OperandSize::Dword)
}

#[test]
fn shr_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 43], OperandSize::Dword)
}

#[test]
fn shr_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 234], OperandSize::Qword)
}

#[test]
fn shr_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RAX, 1641971069, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 168, 125, 125, 222, 97], OperandSize::Qword)
}

#[test]
fn shr_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(RCX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 233], OperandSize::Qword)
}

#[test]
fn shr_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 44, 192], OperandSize::Qword)
}

