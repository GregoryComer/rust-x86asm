use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ror_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 203, 68], OperandSize::Word)
}

#[test]
fn ror_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 251, Some(OperandSize::Byte), None)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 137, 251, 0, 8], OperandSize::Word)
}

#[test]
fn ror_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 201, 115], OperandSize::Dword)
}

#[test]
fn ror_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1661902385, Some(OperandSize::Byte), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 12, 141, 49, 158, 14, 99, 101], OperandSize::Dword)
}

#[test]
fn ror_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 201, 20], OperandSize::Qword)
}

#[test]
fn ror_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 12, 154, 124], OperandSize::Qword)
}

#[test]
fn ror_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 203, 82], OperandSize::Qword)
}

#[test]
fn ror_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1299517498, Some(OperandSize::Byte), None)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 12, 221, 58, 16, 117, 77, 10], OperandSize::Qword)
}

#[test]
fn ror_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 204], OperandSize::Word)
}

#[test]
fn ror_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 182, Some(OperandSize::Word), None)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 137, 182, 0, 2], OperandSize::Word)
}

#[test]
fn ror_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DX)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 202, 79], OperandSize::Dword)
}

#[test]
fn ror_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 11, 69], OperandSize::Dword)
}

#[test]
fn ror_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SP)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 204, 40], OperandSize::Qword)
}

#[test]
fn ror_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 15, 71], OperandSize::Qword)
}

#[test]
fn ror_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 204, 105], OperandSize::Word)
}

#[test]
fn ror_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 8427, Some(OperandSize::Dword), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 139, 235, 32, 110], OperandSize::Word)
}

#[test]
fn ror_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 206, 28], OperandSize::Dword)
}

#[test]
fn ror_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 12, 214, 104], OperandSize::Dword)
}

#[test]
fn ror_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 206, 19], OperandSize::Qword)
}

#[test]
fn ror_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RCX, 698132495, Some(OperandSize::Dword), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 137, 15, 168, 156, 41, 110], OperandSize::Qword)
}

#[test]
fn ror_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RSP)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 204, 56], OperandSize::Qword)
}

#[test]
fn ror_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RDI, 1334121936, Some(OperandSize::Qword), None)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 143, 208, 21, 133, 79, 86], OperandSize::Qword)
}

#[test]
fn ror_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 201], OperandSize::Word)
}

#[test]
fn ror_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(BP, 226, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 142, 226, 0], OperandSize::Word)
}

#[test]
fn ror_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 201], OperandSize::Dword)
}

#[test]
fn ror_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 14], OperandSize::Dword)
}

#[test]
fn ror_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 202], OperandSize::Qword)
}

#[test]
fn ror_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 12, 210], OperandSize::Qword)
}

#[test]
fn ror_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 203], OperandSize::Qword)
}

#[test]
fn ror_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 12, 137], OperandSize::Qword)
}

#[test]
fn ror_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 203], OperandSize::Word)
}

#[test]
fn ror_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 120, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 75, 120], OperandSize::Word)
}

#[test]
fn ror_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 203], OperandSize::Dword)
}

#[test]
fn ror_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1818423675, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 140, 129, 123, 241, 98, 108], OperandSize::Dword)
}

#[test]
fn ror_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 206], OperandSize::Qword)
}

#[test]
fn ror_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 15], OperandSize::Qword)
}

#[test]
fn ror_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 206], OperandSize::Word)
}

#[test]
fn ror_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(BP, 198, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 142, 198, 0], OperandSize::Word)
}

#[test]
fn ror_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 202], OperandSize::Dword)
}

#[test]
fn ror_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(EAX, Four, 638552768, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 12, 133, 192, 138, 15, 38], OperandSize::Dword)
}

#[test]
fn ror_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 203], OperandSize::Qword)
}

#[test]
fn ror_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RAX, 1813470988, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 136, 12, 95, 23, 108], OperandSize::Qword)
}

#[test]
fn ror_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 203], OperandSize::Qword)
}

#[test]
fn ror_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 12, 216], OperandSize::Qword)
}

#[test]
fn ror_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 203], OperandSize::Word)
}

#[test]
fn ror_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 31729, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 137, 241, 123], OperandSize::Word)
}

#[test]
fn ror_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 202], OperandSize::Dword)
}

#[test]
fn ror_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(ECX, 1395242905, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 137, 153, 183, 41, 83], OperandSize::Dword)
}

#[test]
fn ror_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 201], OperandSize::Qword)
}

#[test]
fn ror_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1054495554, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 12, 197, 66, 83, 218, 62], OperandSize::Qword)
}

#[test]
fn ror_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 203], OperandSize::Qword)
}

#[test]
fn ror_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 210853799, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 140, 122, 167, 95, 145, 12], OperandSize::Qword)
}

#[test]
fn ror_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 204], OperandSize::Word)
}

#[test]
fn ror_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 10], OperandSize::Word)
}

#[test]
fn ror_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 206], OperandSize::Dword)
}

#[test]
fn ror_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1585682659, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 12, 125, 227, 152, 131, 94], OperandSize::Dword)
}

#[test]
fn ror_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 201], OperandSize::Qword)
}

#[test]
fn ror_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 866941053, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 12, 213, 125, 120, 172, 51], OperandSize::Qword)
}

#[test]
fn ror_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 201], OperandSize::Word)
}

#[test]
fn ror_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(DI, 40, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 77, 40], OperandSize::Word)
}

#[test]
fn ror_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 203], OperandSize::Dword)
}

#[test]
fn ror_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(ESI, 1597960836, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 142, 132, 242, 62, 95], OperandSize::Dword)
}

#[test]
fn ror_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 206], OperandSize::Qword)
}

#[test]
fn ror_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 12, 89], OperandSize::Qword)
}

#[test]
fn ror_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 202], OperandSize::Qword)
}

#[test]
fn ror_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 12, 250], OperandSize::Qword)
}

