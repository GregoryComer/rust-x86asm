use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 250, 214], OperandSize::Dword)
}

fn vpsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1678416766, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 250, 60, 181, 126, 155, 10, 100], OperandSize::Dword)
}

fn vpsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 250, 236], OperandSize::Qword)
}

fn vpsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1580922106, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 250, 140, 127, 250, 244, 58, 94], OperandSize::Qword)
}

fn vpsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 250, 220], OperandSize::Dword)
}

fn vpsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 622164153, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 250, 20, 181, 185, 120, 21, 37], OperandSize::Dword)
}

fn vpsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 250, 209], OperandSize::Qword)
}

fn vpsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 250, 44, 218], OperandSize::Qword)
}

