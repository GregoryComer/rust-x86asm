use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 245, 51], OperandSize::Dword)
}

#[test]
fn vpshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1548062786, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 180, 150, 66, 144, 69, 92, 87], OperandSize::Dword)
}

#[test]
fn vpshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 247, 97], OperandSize::Qword)
}

#[test]
fn vpshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 446606246, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 60, 149, 166, 171, 158, 26, 87], OperandSize::Qword)
}

#[test]
fn vpshuflw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 247, 86], OperandSize::Dword)
}

#[test]
fn vpshuflw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 23, 122], OperandSize::Dword)
}

#[test]
fn vpshuflw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 224, 122], OperandSize::Qword)
}

#[test]
fn vpshuflw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 49, 96], OperandSize::Qword)
}

#[test]
fn vpshuflw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 143, 112, 215, 56], OperandSize::Dword)
}

#[test]
fn vpshuflw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1422158232, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 139, 112, 188, 67, 152, 105, 196, 84, 18], OperandSize::Dword)
}

#[test]
fn vpshuflw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM18)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 127, 140, 112, 210, 117], OperandSize::Qword)
}

#[test]
fn vpshuflw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 218636024, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 143, 112, 44, 253, 248, 30, 8, 13, 104], OperandSize::Qword)
}

#[test]
fn vpshuflw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 173, 112, 239, 48], OperandSize::Dword)
}

#[test]
fn vpshuflw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 170, 112, 52, 216, 120], OperandSize::Dword)
}

#[test]
fn vpshuflw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM16)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 127, 175, 112, 216, 20], OperandSize::Qword)
}

#[test]
fn vpshuflw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1156788021, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 127, 172, 112, 12, 253, 53, 47, 243, 68, 124], OperandSize::Qword)
}

#[test]
fn vpshuflw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 207, 112, 251, 108], OperandSize::Dword)
}

#[test]
fn vpshuflw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(ESI, 1950280633, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 203, 112, 190, 185, 235, 62, 116, 46], OperandSize::Dword)
}

#[test]
fn vpshuflw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM30)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 127, 202, 112, 230, 124], OperandSize::Qword)
}

#[test]
fn vpshuflw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM28)), operand2: Some(IndirectDisplaced(RDX, 1396114471, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 127, 201, 112, 162, 39, 4, 55, 83, 31], OperandSize::Qword)
}

