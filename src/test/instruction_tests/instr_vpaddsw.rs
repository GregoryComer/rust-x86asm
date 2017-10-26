use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 237, 211], OperandSize::Dword)
}

#[test]
fn vpaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 319198478, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 237, 128, 14, 149, 6, 19], OperandSize::Dword)
}

#[test]
fn vpaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 237, 253], OperandSize::Qword)
}

#[test]
fn vpaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1439336125, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 237, 12, 245, 189, 134, 202, 85], OperandSize::Qword)
}

#[test]
fn vpaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 237, 210], OperandSize::Dword)
}

#[test]
fn vpaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EAX, 1558199501, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 237, 168, 205, 60, 224, 92], OperandSize::Dword)
}

#[test]
fn vpaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 237, 224], OperandSize::Qword)
}

#[test]
fn vpaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 237, 1], OperandSize::Qword)
}

#[test]
fn vpaddsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 237, 211], OperandSize::Dword)
}

#[test]
fn vpaddsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 262659307, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 237, 153, 235, 220, 167, 15], OperandSize::Dword)
}

#[test]
fn vpaddsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 143, 237, 195], OperandSize::Qword)
}

#[test]
fn vpaddsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 13, 142, 237, 20, 144], OperandSize::Qword)
}

#[test]
fn vpaddsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 237, 223], OperandSize::Dword)
}

#[test]
fn vpaddsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 172, 237, 60, 199], OperandSize::Dword)
}

#[test]
fn vpaddsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 69, 164, 237, 235], OperandSize::Qword)
}

#[test]
fn vpaddsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1735087486, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 164, 237, 20, 205, 126, 85, 107, 103], OperandSize::Qword)
}

#[test]
fn vpaddsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 205, 237, 215], OperandSize::Dword)
}

#[test]
fn vpaddsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 26350617, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 206, 237, 140, 91, 25, 20, 146, 1], OperandSize::Dword)
}

#[test]
fn vpaddsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 125, 195, 237, 248], OperandSize::Qword)
}

#[test]
fn vpaddsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 641384527, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 21, 198, 237, 148, 138, 79, 192, 58, 38], OperandSize::Qword)
}

