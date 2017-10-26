use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 216, 237], OperandSize::Dword)
}

#[test]
fn vpsubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 440418629, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 216, 153, 69, 65, 64, 26], OperandSize::Dword)
}

#[test]
fn vpsubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 216, 245], OperandSize::Qword)
}

#[test]
fn vpsubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 216, 60, 65], OperandSize::Qword)
}

#[test]
fn vpsubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 216, 223], OperandSize::Dword)
}

#[test]
fn vpsubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1011110058, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 216, 52, 85, 170, 80, 68, 60], OperandSize::Dword)
}

#[test]
fn vpsubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 216, 224], OperandSize::Qword)
}

#[test]
fn vpsubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RSI, 735964976, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 216, 190, 48, 239, 221, 43], OperandSize::Qword)
}

#[test]
fn vpsubusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 216, 223], OperandSize::Dword)
}

#[test]
fn vpsubusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 665741771, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 216, 132, 217, 203, 105, 174, 39], OperandSize::Dword)
}

#[test]
fn vpsubusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 29, 139, 216, 205], OperandSize::Qword)
}

#[test]
fn vpsubusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 125, 142, 216, 59], OperandSize::Qword)
}

#[test]
fn vpsubusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 174, 216, 247], OperandSize::Dword)
}

#[test]
fn vpsubusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 11123077, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 175, 216, 140, 147, 133, 185, 169, 0], OperandSize::Dword)
}

#[test]
fn vpsubusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 117, 170, 216, 192], OperandSize::Qword)
}

#[test]
fn vpsubusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1148923457, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 175, 216, 60, 117, 65, 46, 123, 68], OperandSize::Qword)
}

#[test]
fn vpsubusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 207, 216, 245], OperandSize::Dword)
}

#[test]
fn vpsubusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1103514894, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 201, 216, 164, 208, 14, 77, 198, 65], OperandSize::Dword)
}

#[test]
fn vpsubusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 37, 204, 216, 201], OperandSize::Qword)
}

#[test]
fn vpsubusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 117, 194, 216, 52, 87], OperandSize::Qword)
}

