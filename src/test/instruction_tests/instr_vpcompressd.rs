use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcompressd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 139, 246], OperandSize::Dword)
}

#[test]
fn vpcompressd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 892540592, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 139, 4, 213, 176, 22, 51, 53], OperandSize::Dword)
}

#[test]
fn vpcompressd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 125, 143, 139, 193], OperandSize::Qword)
}

#[test]
fn vpcompressd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectDisplaced(RSI, 1729327728, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 125, 8, 139, 134, 112, 114, 19, 103], OperandSize::Qword)
}

#[test]
fn vpcompressd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 139, 226], OperandSize::Dword)
}

#[test]
fn vpcompressd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectDisplaced(EAX, 1512727877, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 139, 168, 69, 101, 42, 90], OperandSize::Dword)
}

#[test]
fn vpcompressd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 125, 172, 139, 210], OperandSize::Qword)
}

#[test]
fn vpcompressd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectDisplaced(RSI, 595704905, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 139, 190, 73, 188, 129, 35], OperandSize::Qword)
}

#[test]
fn vpcompressd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 139, 200], OperandSize::Dword)
}

#[test]
fn vpcompressd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 139, 10], OperandSize::Dword)
}

#[test]
fn vpcompressd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 139, 219], OperandSize::Qword)
}

#[test]
fn vpcompressd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledDisplaced(RCX, Four, 32993110, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 72, 139, 36, 141, 86, 111, 247, 1], OperandSize::Qword)
}

