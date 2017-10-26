use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 248, 240], OperandSize::Dword)
}

#[test]
fn vpsubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1016106367, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 248, 60, 245, 127, 141, 144, 60], OperandSize::Dword)
}

#[test]
fn vpsubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 248, 244], OperandSize::Qword)
}

#[test]
fn vpsubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 815358040, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 248, 12, 125, 88, 96, 153, 48], OperandSize::Qword)
}

#[test]
fn vpsubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 248, 220], OperandSize::Dword)
}

#[test]
fn vpsubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 1119629842, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 248, 162, 18, 50, 188, 66], OperandSize::Dword)
}

#[test]
fn vpsubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 248, 214], OperandSize::Qword)
}

#[test]
fn vpsubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1713195027, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 248, 172, 82, 19, 72, 29, 102], OperandSize::Qword)
}

#[test]
fn vpsubb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 248, 241], OperandSize::Dword)
}

#[test]
fn vpsubb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 1968552171, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 248, 158, 235, 184, 85, 117], OperandSize::Dword)
}

#[test]
fn vpsubb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 21, 133, 248, 239], OperandSize::Qword)
}

#[test]
fn vpsubb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1384659237, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 125, 131, 248, 132, 67, 37, 57, 136, 82], OperandSize::Qword)
}

#[test]
fn vpsubb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 174, 248, 252], OperandSize::Dword)
}

#[test]
fn vpsubb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 698819601, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 175, 248, 148, 219, 17, 36, 167, 41], OperandSize::Dword)
}

#[test]
fn vpsubb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 5, 170, 248, 197], OperandSize::Qword)
}

#[test]
fn vpsubb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1871292427, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 248, 164, 206, 11, 168, 137, 111], OperandSize::Qword)
}

#[test]
fn vpsubb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 205, 248, 245], OperandSize::Dword)
}

#[test]
fn vpsubb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 45434148, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 248, 36, 125, 36, 69, 181, 2], OperandSize::Dword)
}

#[test]
fn vpsubb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 125, 201, 248, 216], OperandSize::Qword)
}

#[test]
fn vpsubb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1117186120, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 77, 197, 248, 164, 177, 72, 232, 150, 66], OperandSize::Qword)
}

