use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpacksswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 99, 228], OperandSize::Dword)
}

#[test]
fn vpacksswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1979350730, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 99, 188, 83, 202, 126, 250, 117], OperandSize::Dword)
}

#[test]
fn vpacksswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 99, 254], OperandSize::Qword)
}

#[test]
fn vpacksswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 99, 48], OperandSize::Qword)
}

#[test]
fn vpacksswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 99, 208], OperandSize::Dword)
}

#[test]
fn vpacksswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 99, 51], OperandSize::Dword)
}

#[test]
fn vpacksswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 99, 216], OperandSize::Qword)
}

#[test]
fn vpacksswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 99, 52, 182], OperandSize::Qword)
}

#[test]
fn vpacksswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 137, 99, 216], OperandSize::Dword)
}

#[test]
fn vpacksswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 344402637, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 99, 180, 131, 205, 42, 135, 20], OperandSize::Dword)
}

#[test]
fn vpacksswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 21, 137, 99, 195], OperandSize::Qword)
}

#[test]
fn vpacksswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RBX, 1939442358, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 21, 137, 99, 155, 182, 138, 153, 115], OperandSize::Qword)
}

#[test]
fn vpacksswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 174, 99, 221], OperandSize::Dword)
}

#[test]
fn vpacksswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 1252504618, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 169, 99, 172, 91, 42, 180, 167, 74], OperandSize::Dword)
}

#[test]
fn vpacksswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 117, 161, 99, 229], OperandSize::Qword)
}

#[test]
fn vpacksswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 99, 35], OperandSize::Qword)
}

#[test]
fn vpacksswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 207, 99, 207], OperandSize::Dword)
}

#[test]
fn vpacksswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1354084376, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 206, 99, 4, 253, 24, 176, 181, 80], OperandSize::Dword)
}

#[test]
fn vpacksswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 117, 201, 99, 235], OperandSize::Qword)
}

#[test]
fn vpacksswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 2041419716, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 45, 203, 99, 140, 158, 196, 151, 173, 121], OperandSize::Qword)
}

