use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn rsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 229], OperandSize::Dword)
}

fn rsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 42], OperandSize::Dword)
}

fn rsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 223], OperandSize::Qword)
}

fn rsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 7], OperandSize::Qword)
}

