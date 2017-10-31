use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 206, 99], OperandSize::Word)
}

#[test]
fn shld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(DI, 6877, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 181, 221, 26, 37], OperandSize::Word)
}

#[test]
fn shld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 204, 91], OperandSize::Dword)
}

#[test]
fn shld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(ECX, 1541957103, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 185, 239, 101, 232, 91, 111], OperandSize::Dword)
}

#[test]
fn shld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 244, 41], OperandSize::Qword)
}

#[test]
fn shld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 602166886, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 60, 213, 102, 86, 228, 35, 22], OperandSize::Qword)
}

#[test]
fn shld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 247, 61], OperandSize::Word)
}

#[test]
fn shld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 11545, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 139, 25, 45, 98], OperandSize::Word)
}

#[test]
fn shld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 214, 124], OperandSize::Dword)
}

#[test]
fn shld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1317733747, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 188, 115, 115, 5, 139, 78, 1], OperandSize::Dword)
}

#[test]
fn shld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 244, 126], OperandSize::Qword)
}

#[test]
fn shld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(RCX, 441459625, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 177, 169, 35, 80, 26, 96], OperandSize::Qword)
}

#[test]
fn shld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBX)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 220, 93], OperandSize::Qword)
}

#[test]
fn shld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 44, 135, 123], OperandSize::Qword)
}

#[test]
fn shld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 254], OperandSize::Word)
}

#[test]
fn shld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(SI, 140, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 180, 140, 0], OperandSize::Word)
}

#[test]
fn shld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 253], OperandSize::Dword)
}

#[test]
fn shld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 35], OperandSize::Dword)
}

#[test]
fn shld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 251], OperandSize::Qword)
}

#[test]
fn shld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 60, 65], OperandSize::Qword)
}

#[test]
fn shld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 218], OperandSize::Word)
}

#[test]
fn shld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 9138, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 176, 178, 35], OperandSize::Word)
}

#[test]
fn shld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 239], OperandSize::Dword)
}

#[test]
fn shld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(ECX, Two, 475569339, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 52, 77, 187, 156, 88, 28], OperandSize::Dword)
}

#[test]
fn shld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 255], OperandSize::Qword)
}

#[test]
fn shld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(RDI, Four, 290164456, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 44, 189, 232, 142, 75, 17], OperandSize::Qword)
}

#[test]
fn shld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 251], OperandSize::Qword)
}

#[test]
fn shld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 321748269, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 172, 121, 45, 125, 45, 19], OperandSize::Qword)
}

