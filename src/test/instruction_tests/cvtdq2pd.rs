use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 254], OperandSize::Dword)
}

fn cvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 28, 222], OperandSize::Dword)
}

fn cvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 204], OperandSize::Qword)
}

fn cvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 230, 4, 118], OperandSize::Qword)
}

