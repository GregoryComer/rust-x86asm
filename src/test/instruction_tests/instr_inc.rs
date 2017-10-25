use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn inc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[70], OperandSize::Word)
}

#[test]
fn inc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 71], OperandSize::Dword)
}

#[test]
fn inc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 68], OperandSize::Word)
}

#[test]
fn inc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[67], OperandSize::Dword)
}

#[test]
fn inc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 193], OperandSize::Word)
}

#[test]
fn inc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 1], OperandSize::Word)
}

#[test]
fn inc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 195], OperandSize::Dword)
}

#[test]
fn inc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 392754140, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 132, 65, 220, 243, 104, 23], OperandSize::Dword)
}

#[test]
fn inc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 193], OperandSize::Qword)
}

#[test]
fn inc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 4, 143], OperandSize::Qword)
}

#[test]
fn inc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 193], OperandSize::Qword)
}

#[test]
fn inc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledDisplaced(RSI, Two, 1079123580, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 4, 117, 124, 30, 82, 64], OperandSize::Qword)
}

#[test]
fn inc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[71], OperandSize::Word)
}

#[test]
fn inc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Memory(13369, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 6, 57, 52], OperandSize::Word)
}

#[test]
fn inc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 68], OperandSize::Dword)
}

#[test]
fn inc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 4, 251], OperandSize::Dword)
}

#[test]
fn inc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 198], OperandSize::Qword)
}

#[test]
fn inc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(RDX, 1050986845, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 130, 93, 201, 164, 62], OperandSize::Qword)
}

#[test]
fn inc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 71], OperandSize::Word)
}

#[test]
fn inc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 2], OperandSize::Word)
}

#[test]
fn inc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[67], OperandSize::Dword)
}

#[test]
fn inc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 847435102, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 4, 253, 94, 213, 130, 50], OperandSize::Dword)
}

#[test]
fn inc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 197], OperandSize::Qword)
}

#[test]
fn inc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(RBX, 144741580, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 131, 204, 148, 160, 8], OperandSize::Qword)
}

#[test]
fn inc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(RBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 197], OperandSize::Qword)
}

#[test]
fn inc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(RDI, 268770080, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 135, 32, 27, 5, 16], OperandSize::Qword)
}

