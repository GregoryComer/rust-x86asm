use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexp2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 155, 200, 208], OperandSize::Dword)
}

#[test]
fn vexp2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EAX, 1690509511, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 200, 184, 199, 32, 195, 100], OperandSize::Dword)
}

#[test]
fn vexp2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 200, 22], OperandSize::Dword)
}

#[test]
fn vexp2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 253, 157, 200, 238], OperandSize::Qword)
}

#[test]
fn vexp2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM24)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 253, 203, 200, 1], OperandSize::Qword)
}

#[test]
fn vexp2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1003315490, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 218, 200, 60, 93, 34, 97, 205, 59], OperandSize::Qword)
}

