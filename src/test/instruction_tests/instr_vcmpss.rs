use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 194, 240, 36], OperandSize::Dword)
}

#[test]
fn vcmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 194, 44, 130, 80], OperandSize::Dword)
}

#[test]
fn vcmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 194, 228, 17], OperandSize::Qword)
}

#[test]
fn vcmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 194, 60, 186, 90], OperandSize::Qword)
}

#[test]
fn vcmpss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 86, 25, 194, 238, 126], OperandSize::Dword)
}

#[test]
fn vcmpss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1409334640, Some(OperandSize::Dword), None)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 110, 15, 194, 163, 112, 189, 0, 84, 32], OperandSize::Dword)
}

#[test]
fn vcmpss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM23)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 110, 30, 194, 223, 92], OperandSize::Qword)
}

#[test]
fn vcmpss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1529892259, Some(OperandSize::Dword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 102, 13, 194, 20, 245, 163, 77, 48, 91, 13], OperandSize::Qword)
}

