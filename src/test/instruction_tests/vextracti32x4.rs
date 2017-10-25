use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vextracti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 202, 57, 197, 65], OperandSize::Dword)
}

fn vextracti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1098025544, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 57, 12, 213, 72, 138, 114, 65, 15], OperandSize::Dword)
}

fn vextracti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Direct(XMM30)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 147, 125, 201, 57, 238, 61], OperandSize::Qword)
}

fn vextracti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI32x4, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 57, 49, 33], OperandSize::Qword)
}

