use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 61, 234], OperandSize::Dword)
}

#[test]
fn vpmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1278906403, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 61, 36, 149, 35, 144, 58, 76], OperandSize::Dword)
}

#[test]
fn vpmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 61, 198], OperandSize::Qword)
}

#[test]
fn vpmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 61, 28, 178], OperandSize::Qword)
}

#[test]
fn vpmaxsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 61, 211], OperandSize::Dword)
}

#[test]
fn vpmaxsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 61, 42], OperandSize::Dword)
}

#[test]
fn vpmaxsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 61, 217], OperandSize::Qword)
}

#[test]
fn vpmaxsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1820801001, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 61, 52, 189, 233, 55, 135, 108], OperandSize::Qword)
}

#[test]
fn vpmaxsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 61, 249], OperandSize::Dword)
}

#[test]
fn vpmaxsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 461956077, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 61, 140, 66, 237, 227, 136, 27], OperandSize::Dword)
}

#[test]
fn vpmaxsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 1318685797, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 154, 61, 154, 101, 140, 153, 78], OperandSize::Dword)
}

#[test]
fn vpmaxsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 45, 141, 61, 254], OperandSize::Qword)
}

#[test]
fn vpmaxsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RCX, 745580360, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 5, 135, 61, 137, 72, 167, 112, 44], OperandSize::Qword)
}

#[test]
fn vpmaxsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 691576537, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 13, 149, 61, 28, 117, 217, 158, 56, 41], OperandSize::Qword)
}

