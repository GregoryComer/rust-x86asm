use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 76, 216], OperandSize::Dword)
}

#[test]
fn vrcp14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1408206840, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 76, 156, 254, 248, 135, 239, 83], OperandSize::Dword)
}

#[test]
fn vrcp14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 156, 76, 35], OperandSize::Dword)
}

#[test]
fn vrcp14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 253, 143, 76, 249], OperandSize::Qword)
}

#[test]
fn vrcp14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM10)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 137, 76, 22], OperandSize::Qword)
}

#[test]
fn vrcp14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM24)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 253, 154, 76, 0], OperandSize::Qword)
}

#[test]
fn vrcp14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 76, 234], OperandSize::Dword)
}

#[test]
fn vrcp14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 76, 0], OperandSize::Dword)
}

#[test]
fn vrcp14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 844389981, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 189, 76, 52, 69, 93, 94, 84, 50], OperandSize::Dword)
}

#[test]
fn vrcp14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 253, 171, 76, 214], OperandSize::Qword)
}

#[test]
fn vrcp14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1938340314, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 76, 164, 70, 218, 185, 136, 115], OperandSize::Qword)
}

#[test]
fn vrcp14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 2059397799, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 253, 189, 76, 60, 205, 167, 234, 191, 122], OperandSize::Qword)
}

#[test]
fn vrcp14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 76, 248], OperandSize::Dword)
}

#[test]
fn vrcp14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 76, 30], OperandSize::Dword)
}

#[test]
fn vrcp14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1582918419, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 223, 76, 140, 147, 19, 107, 89, 94], OperandSize::Dword)
}

#[test]
fn vrcp14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 76, 202], OperandSize::Qword)
}

#[test]
fn vrcp14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(RCX, 921661247, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 76, 153, 63, 111, 239, 54], OperandSize::Qword)
}

#[test]
fn vrcp14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1102858880, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 223, 76, 60, 197, 128, 74, 188, 65], OperandSize::Qword)
}

