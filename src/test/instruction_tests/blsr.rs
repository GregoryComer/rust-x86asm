use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn blsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 243, 205], OperandSize::Dword)
}

fn blsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(EBX, 1117075621, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 243, 139, 165, 56, 149, 66], OperandSize::Dword)
}

fn blsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 203], OperandSize::Qword)
}

fn blsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 12, 250], OperandSize::Qword)
}

fn blsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 243, 202], OperandSize::Qword)
}

fn blsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSR, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1892439081, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 243, 12, 253, 41, 84, 204, 112], OperandSize::Qword)
}

