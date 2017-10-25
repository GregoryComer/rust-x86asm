use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 22, 234], OperandSize::Dword)
}

#[test]
fn vpermps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 22, 4, 223], OperandSize::Dword)
}

#[test]
fn vpermps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 22, 215], OperandSize::Qword)
}

#[test]
fn vpermps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 973628589, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 22, 60, 157, 173, 100, 8, 58], OperandSize::Qword)
}

#[test]
fn vpermps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 22, 251], OperandSize::Dword)
}

#[test]
fn vpermps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EBX, 2042291712, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 22, 147, 0, 230, 186, 121], OperandSize::Dword)
}

#[test]
fn vpermps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 190, 22, 2], OperandSize::Dword)
}

#[test]
fn vpermps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 77, 162, 22, 224], OperandSize::Qword)
}

#[test]
fn vpermps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 29, 163, 22, 54], OperandSize::Qword)
}

#[test]
fn vpermps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1052503805, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 125, 182, 22, 140, 241, 253, 238, 187, 62], OperandSize::Qword)
}

#[test]
fn vpermps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 22, 234], OperandSize::Dword)
}

#[test]
fn vpermps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 22, 20, 135], OperandSize::Dword)
}

#[test]
fn vpermps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 217, 22, 4, 216], OperandSize::Dword)
}

#[test]
fn vpermps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 125, 195, 22, 194], OperandSize::Qword)
}

#[test]
fn vpermps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 196, 22, 52, 209], OperandSize::Qword)
}

#[test]
fn vpermps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1163026161, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 215, 22, 164, 87, 241, 94, 82, 69], OperandSize::Qword)
}

