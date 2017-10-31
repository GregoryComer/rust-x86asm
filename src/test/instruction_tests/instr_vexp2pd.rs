use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexp2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 156, 200, 220], OperandSize::Dword)
}

#[test]
fn vexp2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 200, 56], OperandSize::Dword)
}

#[test]
fn vexp2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1373316497, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 200, 44, 197, 145, 37, 219, 81], OperandSize::Dword)
}

#[test]
fn vexp2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 253, 155, 200, 229], OperandSize::Qword)
}

#[test]
fn vexp2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 200, 36, 115], OperandSize::Qword)
}

#[test]
fn vexp2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXP2PD, operand1: Some(Direct(ZMM15)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 218, 200, 58], OperandSize::Qword)
}

