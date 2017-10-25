use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vinserti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 56, 226, 40], OperandSize::Dword)
}

fn vinserti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 56, 24, 5], OperandSize::Dword)
}

fn vinserti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 56, 200, 119], OperandSize::Qword)
}

fn vinserti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTI128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 56, 4, 115, 83], OperandSize::Qword)
}

