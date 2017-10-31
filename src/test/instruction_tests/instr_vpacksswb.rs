use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpacksswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 99, 243], OperandSize::Dword)
}

#[test]
fn vpacksswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 1713228967, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 99, 161, 167, 204, 29, 102], OperandSize::Dword)
}

#[test]
fn vpacksswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 99, 239], OperandSize::Qword)
}

#[test]
fn vpacksswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDX, 1837276550, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 99, 130, 134, 157, 130, 109], OperandSize::Qword)
}

#[test]
fn vpacksswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 99, 197], OperandSize::Dword)
}

#[test]
fn vpacksswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 99, 59], OperandSize::Dword)
}

#[test]
fn vpacksswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 99, 250], OperandSize::Qword)
}

#[test]
fn vpacksswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 99, 4, 147], OperandSize::Qword)
}

#[test]
fn vpacksswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 137, 99, 249], OperandSize::Dword)
}

#[test]
fn vpacksswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 2136829600, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 139, 99, 20, 245, 160, 110, 93, 127], OperandSize::Dword)
}

#[test]
fn vpacksswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 117, 139, 99, 211], OperandSize::Qword)
}

#[test]
fn vpacksswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 363796273, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 69, 142, 99, 28, 205, 49, 23, 175, 21], OperandSize::Qword)
}

#[test]
fn vpacksswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 174, 99, 201], OperandSize::Dword)
}

#[test]
fn vpacksswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 865535595, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 99, 188, 79, 107, 6, 151, 51], OperandSize::Dword)
}

#[test]
fn vpacksswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 13, 161, 99, 213], OperandSize::Qword)
}

#[test]
fn vpacksswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 172, 99, 20, 152], OperandSize::Qword)
}

#[test]
fn vpacksswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 99, 204], OperandSize::Dword)
}

#[test]
fn vpacksswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ECX, 856135586, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 207, 99, 177, 162, 151, 7, 51], OperandSize::Dword)
}

#[test]
fn vpacksswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 37, 196, 99, 255], OperandSize::Qword)
}

#[test]
fn vpacksswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1116888545, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 53, 197, 99, 52, 85, 225, 93, 146, 66], OperandSize::Qword)
}

