use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 153, 204, 250], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 204, 20, 210], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 531262070, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 204, 188, 137, 118, 106, 170, 31], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 159, 204, 193], OperandSize::Qword)
}

#[test]
fn vrsqrt28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 201, 204, 44, 207], OperandSize::Qword)
}

#[test]
fn vrsqrt28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM20)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 222, 204, 38], OperandSize::Qword)
}

