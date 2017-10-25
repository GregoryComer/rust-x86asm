use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 58, 206], OperandSize::Dword)
}

#[test]
fn vpminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1168068227, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 58, 132, 177, 131, 78, 159, 69], OperandSize::Dword)
}

#[test]
fn vpminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 58, 254], OperandSize::Qword)
}

#[test]
fn vpminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1311952437, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 58, 20, 141, 53, 206, 50, 78], OperandSize::Qword)
}

#[test]
fn vpminuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 58, 212], OperandSize::Dword)
}

#[test]
fn vpminuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 1933191489, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 58, 155, 65, 41, 58, 115], OperandSize::Dword)
}

#[test]
fn vpminuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 58, 226], OperandSize::Qword)
}

#[test]
fn vpminuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 58, 27], OperandSize::Qword)
}

#[test]
fn vpminuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 142, 58, 202], OperandSize::Dword)
}

#[test]
fn vpminuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 2142683304, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 58, 144, 168, 192, 182, 127], OperandSize::Dword)
}

#[test]
fn vpminuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 45, 138, 58, 255], OperandSize::Qword)
}

#[test]
fn vpminuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RCX, 1935665662, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 58, 137, 254, 233, 95, 115], OperandSize::Qword)
}

#[test]
fn vpminuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 172, 58, 230], OperandSize::Dword)
}

#[test]
fn vpminuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 359807270, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 174, 58, 28, 149, 38, 57, 114, 21], OperandSize::Dword)
}

#[test]
fn vpminuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 29, 173, 58, 215], OperandSize::Qword)
}

#[test]
fn vpminuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 29, 175, 58, 20, 201], OperandSize::Qword)
}

#[test]
fn vpminuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 202, 58, 253], OperandSize::Dword)
}

#[test]
fn vpminuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 201, 58, 38], OperandSize::Dword)
}

#[test]
fn vpminuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 109, 207, 58, 239], OperandSize::Qword)
}

#[test]
fn vpminuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectDisplaced(RDI, 1489624231, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 21, 207, 58, 143, 167, 220, 201, 88], OperandSize::Qword)
}

