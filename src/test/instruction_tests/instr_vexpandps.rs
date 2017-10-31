use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexpandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 136, 207], OperandSize::Dword)
}

#[test]
fn vexpandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 996018136, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 136, 20, 197, 216, 7, 94, 59], OperandSize::Dword)
}

#[test]
fn vexpandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 136, 194], OperandSize::Qword)
}

#[test]
fn vexpandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM25)), operand2: Some(IndirectDisplaced(RSI, 2044936410, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 139, 136, 142, 218, 64, 227, 121], OperandSize::Qword)
}

#[test]
fn vexpandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 136, 210], OperandSize::Dword)
}

#[test]
fn vexpandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 136, 16], OperandSize::Dword)
}

#[test]
fn vexpandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 125, 175, 136, 241], OperandSize::Qword)
}

#[test]
fn vexpandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 169, 136, 36, 176], OperandSize::Qword)
}

#[test]
fn vexpandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 136, 252], OperandSize::Dword)
}

#[test]
fn vexpandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1670137871, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 136, 132, 154, 15, 72, 140, 99], OperandSize::Dword)
}

#[test]
fn vexpandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 207, 136, 192], OperandSize::Qword)
}

#[test]
fn vexpandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 309583949, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 136, 12, 77, 77, 224, 115, 18], OperandSize::Qword)
}

