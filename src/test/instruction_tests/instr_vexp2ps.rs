use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexp2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 157, 200, 248], OperandSize::Dword)
}

#[test]
fn vexp2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 1734063493, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 200, 140, 74, 133, 181, 91, 103], OperandSize::Dword)
}

#[test]
fn vexp2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 200, 46], OperandSize::Dword)
}

#[test]
fn vexp2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 125, 153, 200, 251], OperandSize::Qword)
}

#[test]
fn vexp2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 900102308, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 200, 4, 77, 164, 120, 166, 53], OperandSize::Qword)
}

#[test]
fn vexp2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PS, operand1: Some(Direct(ZMM17)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 221, 200, 9], OperandSize::Qword)
}

