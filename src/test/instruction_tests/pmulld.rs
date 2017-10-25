use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 254], OperandSize::Dword)
}

fn pmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 0], OperandSize::Dword)
}

fn pmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 195], OperandSize::Qword)
}

fn pmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 17], OperandSize::Qword)
}

