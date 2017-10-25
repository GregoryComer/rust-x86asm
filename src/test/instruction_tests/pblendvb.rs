use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pblendvb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 228], OperandSize::Dword)
}

fn pblendvb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 44, 131], OperandSize::Dword)
}

fn pblendvb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 236], OperandSize::Qword)
}

fn pblendvb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDVB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 16, 12, 222], OperandSize::Qword)
}

