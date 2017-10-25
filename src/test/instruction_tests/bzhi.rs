use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bzhi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 245, 219], OperandSize::Dword)
}

fn bzhi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 343216094, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 245, 20, 221, 222, 15, 117, 20], OperandSize::Dword)
}

fn bzhi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 245, 233], OperandSize::Qword)
}

fn bzhi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1656634637, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 245, 44, 205, 13, 61, 190, 98], OperandSize::Qword)
}

fn bzhi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSP)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 232, 245, 204], OperandSize::Qword)
}

fn bzhi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BZHI, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RCX, 2128562183, Some(OperandSize::Qword), None)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 224, 245, 153, 7, 72, 223, 126], OperandSize::Qword)
}

