use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn inc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[67], OperandSize::Word)
}

#[test]
fn inc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 71], OperandSize::Dword)
}

#[test]
fn inc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 70], OperandSize::Word)
}

#[test]
fn inc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[69], OperandSize::Dword)
}

#[test]
fn inc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 195], OperandSize::Word)
}

#[test]
fn inc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(BX, 26199, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 135, 87, 102], OperandSize::Word)
}

#[test]
fn inc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 195], OperandSize::Dword)
}

#[test]
fn inc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(EDI, 667924827, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 135, 91, 185, 207, 39], OperandSize::Dword)
}

#[test]
fn inc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 195], OperandSize::Qword)
}

#[test]
fn inc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 7], OperandSize::Qword)
}

#[test]
fn inc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 193], OperandSize::Qword)
}

#[test]
fn inc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 4, 242], OperandSize::Qword)
}

#[test]
fn inc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[71], OperandSize::Word)
}

#[test]
fn inc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(DI, 83, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 69, 83], OperandSize::Word)
}

#[test]
fn inc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 71], OperandSize::Dword)
}

#[test]
fn inc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledDisplaced(ECX, Two, 471944862, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 4, 77, 158, 78, 33, 28], OperandSize::Dword)
}

#[test]
fn inc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 199], OperandSize::Qword)
}

#[test]
fn inc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(RSI, 2092911510, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 134, 150, 75, 191, 124], OperandSize::Qword)
}

#[test]
fn inc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 65], OperandSize::Word)
}

#[test]
fn inc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 2], OperandSize::Word)
}

#[test]
fn inc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[69], OperandSize::Dword)
}

#[test]
fn inc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 993733682, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 132, 126, 50, 44, 59, 59], OperandSize::Dword)
}

#[test]
fn inc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 197], OperandSize::Qword)
}

#[test]
fn inc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 170252071, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 132, 178, 39, 215, 37, 10], OperandSize::Qword)
}

#[test]
fn inc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 198], OperandSize::Qword)
}

#[test]
fn inc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 693566626, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 132, 87, 162, 252, 86, 41], OperandSize::Qword)
}

