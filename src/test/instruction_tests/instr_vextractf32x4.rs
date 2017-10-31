use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 204, 25, 213, 78], OperandSize::Dword)
}

#[test]
fn vextractf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 25, 52, 241, 34], OperandSize::Dword)
}

#[test]
fn vextractf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM18)), operand2: Some(Direct(ZMM26)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 35, 125, 206, 25, 210, 14], OperandSize::Qword)
}

#[test]
fn vextractf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM28)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 72, 25, 36, 193, 87], OperandSize::Qword)
}

