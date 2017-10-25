use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 60, 223], OperandSize::Dword)
}

#[test]
fn vpmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 60, 9], OperandSize::Dword)
}

#[test]
fn vpmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 60, 221], OperandSize::Qword)
}

#[test]
fn vpmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 679547839, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 60, 36, 157, 191, 19, 129, 40], OperandSize::Qword)
}

#[test]
fn vpmaxsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 60, 244], OperandSize::Dword)
}

#[test]
fn vpmaxsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 1652914731, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 60, 180, 123, 43, 122, 133, 98], OperandSize::Dword)
}

#[test]
fn vpmaxsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 60, 198], OperandSize::Qword)
}

#[test]
fn vpmaxsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 60, 20, 118], OperandSize::Qword)
}

#[test]
fn vpmaxsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 60, 216], OperandSize::Dword)
}

#[test]
fn vpmaxsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 347686358, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 60, 4, 253, 214, 69, 185, 20], OperandSize::Dword)
}

#[test]
fn vpmaxsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 129, 60, 250], OperandSize::Qword)
}

#[test]
fn vpmaxsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RCX, 1252089786, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 21, 139, 60, 177, 186, 95, 161, 74], OperandSize::Qword)
}

#[test]
fn vpmaxsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 173, 60, 197], OperandSize::Dword)
}

#[test]
fn vpmaxsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 506750161, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 60, 4, 181, 209, 100, 52, 30], OperandSize::Dword)
}

#[test]
fn vpmaxsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 85, 173, 60, 223], OperandSize::Qword)
}

#[test]
fn vpmaxsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1183788657, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 53, 173, 60, 60, 85, 113, 46, 143, 70], OperandSize::Qword)
}

#[test]
fn vpmaxsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 206, 60, 246], OperandSize::Dword)
}

#[test]
fn vpmaxsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 201, 60, 44, 217], OperandSize::Dword)
}

#[test]
fn vpmaxsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 29, 193, 60, 198], OperandSize::Qword)
}

#[test]
fn vpmaxsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 2034767431, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 69, 193, 60, 148, 119, 71, 22, 72, 121], OperandSize::Qword)
}

