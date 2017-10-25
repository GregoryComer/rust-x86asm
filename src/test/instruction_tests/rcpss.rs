use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn rcpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 236], OperandSize::Dword)
}

fn rcpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1764144472, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 148, 193, 88, 181, 38, 105], OperandSize::Dword)
}

fn rcpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 208], OperandSize::Qword)
}

fn rcpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RAX, 1093198751, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 184, 159, 227, 40, 65], OperandSize::Qword)
}

