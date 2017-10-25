use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcompressd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 139, 216], OperandSize::Dword)
}

#[test]
fn vpcompressd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 757410059, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 139, 180, 152, 11, 41, 37, 45], OperandSize::Dword)
}

#[test]
fn vpcompressd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 125, 142, 139, 238], OperandSize::Qword)
}

#[test]
fn vpcompressd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 8, 139, 43], OperandSize::Qword)
}

#[test]
fn vpcompressd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 139, 209], OperandSize::Dword)
}

#[test]
fn vpcompressd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledDisplaced(EAX, Four, 879374216, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 139, 20, 133, 136, 47, 106, 52], OperandSize::Dword)
}

#[test]
fn vpcompressd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 125, 174, 139, 198], OperandSize::Qword)
}

#[test]
fn vpcompressd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 139, 55], OperandSize::Qword)
}

#[test]
fn vpcompressd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 139, 225], OperandSize::Dword)
}

#[test]
fn vpcompressd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 139, 43], OperandSize::Dword)
}

#[test]
fn vpcompressd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 125, 204, 139, 192], OperandSize::Qword)
}

#[test]
fn vpcompressd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 125, 72, 139, 4, 248], OperandSize::Qword)
}

