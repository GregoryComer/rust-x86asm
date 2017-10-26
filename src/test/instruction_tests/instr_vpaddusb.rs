use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 220, 229], OperandSize::Dword)
}

#[test]
fn vpaddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 220, 44, 118], OperandSize::Dword)
}

#[test]
fn vpaddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 220, 247], OperandSize::Qword)
}

#[test]
fn vpaddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 538759150, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 220, 36, 125, 238, 207, 28, 32], OperandSize::Qword)
}

#[test]
fn vpaddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 220, 250], OperandSize::Dword)
}

#[test]
fn vpaddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 220, 44, 139], OperandSize::Dword)
}

#[test]
fn vpaddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 220, 198], OperandSize::Qword)
}

#[test]
fn vpaddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 220, 52, 243], OperandSize::Qword)
}

#[test]
fn vpaddusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 220, 220], OperandSize::Dword)
}

#[test]
fn vpaddusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 1296932176, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 220, 174, 80, 157, 77, 77], OperandSize::Dword)
}

#[test]
fn vpaddusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 117, 135, 220, 196], OperandSize::Qword)
}

#[test]
fn vpaddusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 101, 138, 220, 2], OperandSize::Qword)
}

#[test]
fn vpaddusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 220, 212], OperandSize::Dword)
}

#[test]
fn vpaddusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 200614566, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 220, 180, 241, 166, 34, 245, 11], OperandSize::Dword)
}

#[test]
fn vpaddusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 101, 170, 220, 233], OperandSize::Qword)
}

#[test]
fn vpaddusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 253340975, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 220, 52, 197, 47, 173, 25, 15], OperandSize::Qword)
}

#[test]
fn vpaddusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 220, 198], OperandSize::Dword)
}

#[test]
fn vpaddusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1245857744, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 205, 220, 28, 157, 208, 71, 66, 74], OperandSize::Dword)
}

#[test]
fn vpaddusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 37, 201, 220, 250], OperandSize::Qword)
}

#[test]
fn vpaddusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 29, 195, 220, 63], OperandSize::Qword)
}

