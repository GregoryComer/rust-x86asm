use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaddsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 208, 219], OperandSize::Dword)
}

fn vaddsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 820024100, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 208, 167, 36, 147, 224, 48], OperandSize::Dword)
}

fn vaddsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 208, 240], OperandSize::Qword)
}

fn vaddsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 229085579, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 208, 148, 249, 139, 145, 167, 13], OperandSize::Qword)
}

fn vaddsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 208, 207], OperandSize::Dword)
}

fn vaddsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 208, 12, 183], OperandSize::Dword)
}

fn vaddsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 208, 252], OperandSize::Qword)
}

fn vaddsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 208, 49], OperandSize::Qword)
}

