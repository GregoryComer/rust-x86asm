use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vextractf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 232, 76], OperandSize::Dword)
}

fn vextractf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 2, 109], OperandSize::Dword)
}

fn vextractf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 222, 4], OperandSize::Qword)
}

fn vextractf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 12, 222, 75], OperandSize::Qword)
}

