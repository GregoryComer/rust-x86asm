use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcompressd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 139, 223], OperandSize::Dword)
}

#[test]
fn vpcompressd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 434673451, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 139, 188, 87, 43, 151, 232, 25], OperandSize::Dword)
}

#[test]
fn vpcompressd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 142, 139, 207], OperandSize::Qword)
}

#[test]
fn vpcompressd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectDisplaced(RAX, 850695796, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 125, 8, 139, 152, 116, 150, 180, 50], OperandSize::Qword)
}

#[test]
fn vpcompressd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 139, 218], OperandSize::Dword)
}

#[test]
fn vpcompressd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectDisplaced(EBX, 916167210, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 139, 163, 42, 154, 155, 54], OperandSize::Dword)
}

#[test]
fn vpcompressd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 125, 171, 139, 246], OperandSize::Qword)
}

#[test]
fn vpcompressd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 139, 44, 74], OperandSize::Qword)
}

#[test]
fn vpcompressd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 139, 219], OperandSize::Dword)
}

#[test]
fn vpcompressd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectDisplaced(EDX, 412427681, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 139, 162, 161, 37, 149, 24], OperandSize::Dword)
}

#[test]
fn vpcompressd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 125, 202, 139, 250], OperandSize::Qword)
}

#[test]
fn vpcompressd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 72, 139, 44, 120], OperandSize::Qword)
}

