use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 62, 218], OperandSize::Dword)
}

#[test]
fn vpmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 62, 20, 214], OperandSize::Dword)
}

#[test]
fn vpmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 62, 226], OperandSize::Qword)
}

#[test]
fn vpmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1960264288, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 62, 4, 189, 96, 66, 215, 116], OperandSize::Qword)
}

#[test]
fn vpmaxuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 62, 207], OperandSize::Dword)
}

#[test]
fn vpmaxuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 62, 6], OperandSize::Dword)
}

#[test]
fn vpmaxuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 62, 213], OperandSize::Qword)
}

#[test]
fn vpmaxuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 62, 35], OperandSize::Qword)
}

#[test]
fn vpmaxuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 62, 251], OperandSize::Dword)
}

#[test]
fn vpmaxuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 741005103, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 139, 62, 153, 47, 215, 42, 44], OperandSize::Dword)
}

#[test]
fn vpmaxuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 5, 129, 62, 213], OperandSize::Qword)
}

#[test]
fn vpmaxuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 62, 49], OperandSize::Qword)
}

#[test]
fn vpmaxuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 62, 221], OperandSize::Dword)
}

#[test]
fn vpmaxuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 965210784, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 62, 52, 149, 160, 242, 135, 57], OperandSize::Dword)
}

#[test]
fn vpmaxuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 93, 161, 62, 255], OperandSize::Qword)
}

#[test]
fn vpmaxuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 93, 165, 62, 12, 190], OperandSize::Qword)
}

#[test]
fn vpmaxuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 202, 62, 249], OperandSize::Dword)
}

#[test]
fn vpmaxuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 202, 62, 59], OperandSize::Dword)
}

#[test]
fn vpmaxuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 69, 199, 62, 213], OperandSize::Qword)
}

#[test]
fn vpmaxuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 203, 62, 59], OperandSize::Qword)
}

