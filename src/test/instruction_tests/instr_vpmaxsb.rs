use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 60, 248], OperandSize::Dword)
}

#[test]
fn vpmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 60, 52, 70], OperandSize::Dword)
}

#[test]
fn vpmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 60, 229], OperandSize::Qword)
}

#[test]
fn vpmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 60, 20, 201], OperandSize::Qword)
}

#[test]
fn vpmaxsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 60, 202], OperandSize::Dword)
}

#[test]
fn vpmaxsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 60, 60, 179], OperandSize::Dword)
}

#[test]
fn vpmaxsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 60, 222], OperandSize::Qword)
}

#[test]
fn vpmaxsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1598880003, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 60, 36, 189, 3, 249, 76, 95], OperandSize::Qword)
}

#[test]
fn vpmaxsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 60, 192], OperandSize::Dword)
}

#[test]
fn vpmaxsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDX, 274820959, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 137, 60, 178, 95, 111, 97, 16], OperandSize::Dword)
}

#[test]
fn vpmaxsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 85, 138, 60, 207], OperandSize::Qword)
}

#[test]
fn vpmaxsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 320661301, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 37, 135, 60, 172, 217, 53, 231, 28, 19], OperandSize::Qword)
}

#[test]
fn vpmaxsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 60, 209], OperandSize::Dword)
}

#[test]
fn vpmaxsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 170, 60, 36, 250], OperandSize::Dword)
}

#[test]
fn vpmaxsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 69, 173, 60, 207], OperandSize::Qword)
}

#[test]
fn vpmaxsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 727432219, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 69, 171, 60, 188, 80, 27, 188, 91, 43], OperandSize::Qword)
}

#[test]
fn vpmaxsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 60, 205], OperandSize::Dword)
}

#[test]
fn vpmaxsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 203, 60, 12, 185], OperandSize::Dword)
}

#[test]
fn vpmaxsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 45, 202, 60, 250], OperandSize::Qword)
}

#[test]
fn vpmaxsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 93, 195, 60, 44, 74], OperandSize::Qword)
}

