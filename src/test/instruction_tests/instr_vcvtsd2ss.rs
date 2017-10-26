use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 90, 254], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1943779089, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 90, 188, 146, 17, 183, 219, 115], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 90, 223], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1256288661, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 90, 52, 141, 149, 113, 225, 74], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 231, 219, 90, 231], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 1043430813, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 247, 141, 90, 179, 157, 125, 49, 62], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 239, 186, 90, 208], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1271971956, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 191, 130, 90, 20, 125, 116, 192, 208, 75], OperandSize::Qword)
}

