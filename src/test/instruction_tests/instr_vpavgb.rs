use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 224, 213], OperandSize::Dword)
}

#[test]
fn vpavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1394722720, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 224, 36, 213, 160, 199, 33, 83], OperandSize::Dword)
}

#[test]
fn vpavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 224, 198], OperandSize::Qword)
}

#[test]
fn vpavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 488729427, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 224, 180, 147, 83, 107, 33, 29], OperandSize::Qword)
}

#[test]
fn vpavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 224, 249], OperandSize::Dword)
}

#[test]
fn vpavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 740109606, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 224, 60, 181, 38, 45, 29, 44], OperandSize::Dword)
}

#[test]
fn vpavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 224, 201], OperandSize::Qword)
}

#[test]
fn vpavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 224, 10], OperandSize::Qword)
}

#[test]
fn vpavgb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 143, 224, 245], OperandSize::Dword)
}

#[test]
fn vpavgb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 224, 50], OperandSize::Dword)
}

#[test]
fn vpavgb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 93, 142, 224, 240], OperandSize::Qword)
}

#[test]
fn vpavgb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RBX, 600226660, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 29, 139, 224, 171, 100, 187, 198, 35], OperandSize::Qword)
}

#[test]
fn vpavgb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 174, 224, 241], OperandSize::Dword)
}

#[test]
fn vpavgb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 174, 224, 60, 81], OperandSize::Dword)
}

#[test]
fn vpavgb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 45, 166, 224, 254], OperandSize::Qword)
}

#[test]
fn vpavgb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 237165809, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 69, 167, 224, 132, 119, 241, 220, 34, 14], OperandSize::Qword)
}

#[test]
fn vpavgb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 201, 224, 200], OperandSize::Dword)
}

#[test]
fn vpavgb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 45456604, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 224, 148, 123, 220, 156, 181, 2], OperandSize::Dword)
}

#[test]
fn vpavgb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 53, 193, 224, 242], OperandSize::Qword)
}

#[test]
fn vpavgb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RBX, 1151395279, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 69, 196, 224, 131, 207, 229, 160, 68], OperandSize::Qword)
}

