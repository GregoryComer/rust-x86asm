use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 237, 255], OperandSize::Dword)
}

#[test]
fn vpaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 237, 49], OperandSize::Dword)
}

#[test]
fn vpaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 237, 214], OperandSize::Qword)
}

#[test]
fn vpaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1559766801, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 237, 44, 69, 17, 39, 248, 92], OperandSize::Qword)
}

#[test]
fn vpaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 237, 223], OperandSize::Dword)
}

#[test]
fn vpaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 215971132, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 237, 168, 60, 117, 223, 12], OperandSize::Dword)
}

#[test]
fn vpaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 237, 250], OperandSize::Qword)
}

#[test]
fn vpaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1616383323, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 237, 180, 150, 91, 13, 88, 96], OperandSize::Qword)
}

#[test]
fn vpaddsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 237, 248], OperandSize::Dword)
}

#[test]
fn vpaddsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 341518057, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 237, 60, 85, 233, 38, 91, 20], OperandSize::Dword)
}

#[test]
fn vpaddsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 101, 142, 237, 252], OperandSize::Qword)
}

#[test]
fn vpaddsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 650493627, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 117, 131, 237, 28, 245, 187, 190, 197, 38], OperandSize::Qword)
}

#[test]
fn vpaddsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 237, 195], OperandSize::Dword)
}

#[test]
fn vpaddsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 174, 237, 42], OperandSize::Dword)
}

#[test]
fn vpaddsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 69, 161, 237, 243], OperandSize::Qword)
}

#[test]
fn vpaddsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 1280182124, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 29, 164, 237, 164, 78, 108, 7, 78, 76], OperandSize::Qword)
}

#[test]
fn vpaddsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 237, 199], OperandSize::Dword)
}

#[test]
fn vpaddsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1861631705, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 206, 237, 28, 85, 217, 62, 246, 110], OperandSize::Dword)
}

#[test]
fn vpaddsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 125, 206, 237, 212], OperandSize::Qword)
}

#[test]
fn vpaddsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 154177851, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 101, 205, 237, 20, 189, 59, 145, 48, 9], OperandSize::Qword)
}

