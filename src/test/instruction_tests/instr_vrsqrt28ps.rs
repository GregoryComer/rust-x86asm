use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 155, 204, 247], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 204, 51], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1196932834, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 217, 204, 188, 201, 226, 190, 87, 71], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 125, 153, 204, 241], OperandSize::Qword)
}

#[test]
fn vrsqrt28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectDisplaced(RDX, 1927412207, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 206, 204, 186, 239, 249, 225, 114], OperandSize::Qword)
}

#[test]
fn vrsqrt28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 340844091, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 219, 204, 188, 146, 59, 222, 80, 20], OperandSize::Qword)
}

