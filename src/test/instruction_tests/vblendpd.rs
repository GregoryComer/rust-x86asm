use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vblendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 13, 223, 78], OperandSize::Dword)
}

fn vblendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 133484977, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 13, 182, 177, 209, 244, 7, 39], OperandSize::Dword)
}

fn vblendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 13, 250, 80], OperandSize::Qword)
}

fn vblendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 13, 44, 187, 84], OperandSize::Qword)
}

fn vblendpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 13, 194, 85], OperandSize::Dword)
}

fn vblendpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1140452684, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 13, 44, 117, 76, 237, 249, 67, 32], OperandSize::Dword)
}

fn vblendpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 13, 211, 12], OperandSize::Qword)
}

fn vblendpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 13, 38, 28], OperandSize::Qword)
}

