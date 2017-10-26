use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexp2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 153, 200, 202], OperandSize::Dword)
}

#[test]
fn vexp2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 200, 28, 192], OperandSize::Dword)
}

#[test]
fn vexp2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 200, 11], OperandSize::Dword)
}

#[test]
fn vexp2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 155, 200, 232], OperandSize::Qword)
}

#[test]
fn vexp2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM8)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 201, 200, 0], OperandSize::Qword)
}

#[test]
fn vexp2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 466715490, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 222, 200, 36, 77, 98, 131, 209, 27], OperandSize::Qword)
}

