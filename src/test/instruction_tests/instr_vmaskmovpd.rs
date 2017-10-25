use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaskmovpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 655158888, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 45, 174, 104, 238, 12, 39], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 114214034, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 45, 52, 69, 146, 196, 206, 6], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 1771936812, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 45, 131, 44, 156, 157, 105], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1512169647, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 45, 156, 121, 175, 224, 33, 90], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1128082760, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 47, 36, 181, 72, 45, 61, 67], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectDisplaced(RDX, 1240197879, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 47, 138, 247, 234, 235, 73], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1099830953, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 47, 172, 131, 169, 22, 142, 65], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 47, 28, 159], OperandSize::Qword)
}

