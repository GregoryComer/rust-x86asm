use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexp2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 156, 200, 213], OperandSize::Dword)
}

#[test]
fn vexp2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1785421294, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 200, 132, 206, 238, 93, 107, 106], OperandSize::Dword)
}

#[test]
fn vexp2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 200, 36, 159], OperandSize::Dword)
}

#[test]
fn vexp2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 253, 159, 200, 252], OperandSize::Qword)
}

#[test]
fn vexp2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1689836513, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 200, 180, 139, 225, 219, 184, 100], OperandSize::Qword)
}

#[test]
fn vexp2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 796550169, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 220, 200, 180, 178, 25, 100, 122, 47], OperandSize::Qword)
}

