use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lsl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 250], OperandSize::Word)
}

#[test]
fn lsl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 18610, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 154, 178, 72], OperandSize::Word)
}

#[test]
fn lsl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 254], OperandSize::Dword)
}

#[test]
fn lsl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 526953562, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 60, 141, 90, 172, 104, 31], OperandSize::Dword)
}

#[test]
fn lsl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 245], OperandSize::Qword)
}

#[test]
fn lsl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(SI)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 48], OperandSize::Qword)
}

#[test]
fn lsl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 254], OperandSize::Word)
}

#[test]
fn lsl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(BP, 890, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 158, 122, 3], OperandSize::Word)
}

#[test]
fn lsl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 226], OperandSize::Dword)
}

#[test]
fn lsl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(EDI, 1118282369, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 143, 129, 162, 167, 66], OperandSize::Dword)
}

#[test]
fn lsl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 226], OperandSize::Qword)
}

#[test]
fn lsl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RDX, 1650827425, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 178, 161, 160, 101, 98], OperandSize::Qword)
}

#[test]
fn lsl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RSI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 245], OperandSize::Qword)
}

#[test]
fn lsl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RDX, 1650521311, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 146, 223, 244, 96, 98], OperandSize::Qword)
}

