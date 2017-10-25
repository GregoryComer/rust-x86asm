use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpacksswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 99, 209], OperandSize::Dword)
}

#[test]
fn vpacksswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 99, 42], OperandSize::Dword)
}

#[test]
fn vpacksswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 99, 194], OperandSize::Qword)
}

#[test]
fn vpacksswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 99, 55], OperandSize::Qword)
}

#[test]
fn vpacksswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 99, 192], OperandSize::Dword)
}

#[test]
fn vpacksswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1332283279, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 99, 20, 77, 143, 7, 105, 79], OperandSize::Dword)
}

#[test]
fn vpacksswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 99, 202], OperandSize::Qword)
}

#[test]
fn vpacksswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDI, 1515441287, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 99, 183, 135, 204, 83, 90], OperandSize::Qword)
}

#[test]
fn vpacksswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 99, 240], OperandSize::Dword)
}

#[test]
fn vpacksswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 142, 99, 54], OperandSize::Dword)
}

#[test]
fn vpacksswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 69, 134, 99, 224], OperandSize::Qword)
}

#[test]
fn vpacksswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 141, 99, 11], OperandSize::Qword)
}

#[test]
fn vpacksswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 171, 99, 222], OperandSize::Dword)
}

#[test]
fn vpacksswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ECX, 646162811, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 172, 99, 161, 123, 169, 131, 38], OperandSize::Dword)
}

#[test]
fn vpacksswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 53, 166, 99, 235], OperandSize::Qword)
}

#[test]
fn vpacksswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RSI, 518924988, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 125, 165, 99, 190, 188, 42, 238, 30], OperandSize::Qword)
}

#[test]
fn vpacksswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 207, 99, 216], OperandSize::Dword)
}

#[test]
fn vpacksswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 2028925487, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 206, 99, 172, 150, 47, 242, 238, 120], OperandSize::Dword)
}

#[test]
fn vpacksswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 93, 207, 99, 206], OperandSize::Qword)
}

#[test]
fn vpacksswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RBX, 1099351371, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 37, 206, 99, 131, 75, 197, 134, 65], OperandSize::Qword)
}

