use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 76, 251], OperandSize::Dword)
}

#[test]
fn vrcp14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1811903536, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 76, 28, 181, 48, 116, 255, 107], OperandSize::Dword)
}

#[test]
fn vrcp14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 156, 76, 28, 187], OperandSize::Dword)
}

#[test]
fn vrcp14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 138, 76, 200], OperandSize::Qword)
}

#[test]
fn vrcp14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM9)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 659831431, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 140, 76, 12, 157, 135, 58, 84, 39], OperandSize::Qword)
}

#[test]
fn vrcp14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 76, 34], OperandSize::Qword)
}

#[test]
fn vrcp14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 76, 248], OperandSize::Dword)
}

#[test]
fn vrcp14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1868943492, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 76, 164, 199, 132, 208, 101, 111], OperandSize::Dword)
}

#[test]
fn vrcp14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1186609755, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 76, 20, 157, 91, 58, 186, 70], OperandSize::Dword)
}

#[test]
fn vrcp14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 170, 76, 221], OperandSize::Qword)
}

#[test]
fn vrcp14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RCX, 142074020, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 76, 177, 164, 224, 119, 8], OperandSize::Qword)
}

#[test]
fn vrcp14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 190, 76, 36, 115], OperandSize::Qword)
}

#[test]
fn vrcp14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 76, 227], OperandSize::Dword)
}

#[test]
fn vrcp14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ESI, 1270817289, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 76, 158, 9, 34, 191, 75], OperandSize::Dword)
}

#[test]
fn vrcp14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1206930179, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 223, 76, 28, 133, 3, 75, 240, 71], OperandSize::Dword)
}

#[test]
fn vrcp14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 203, 76, 216], OperandSize::Qword)
}

#[test]
fn vrcp14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1728639589, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 201, 76, 36, 221, 101, 242, 8, 103], OperandSize::Qword)
}

#[test]
fn vrcp14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectDisplaced(RDX, 1368262689, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 217, 76, 186, 33, 8, 142, 81], OperandSize::Qword)
}

