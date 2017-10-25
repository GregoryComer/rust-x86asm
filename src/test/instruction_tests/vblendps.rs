use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vblendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 12, 217, 34], OperandSize::Dword)
}

fn vblendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 12, 10, 11], OperandSize::Dword)
}

fn vblendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 12, 210, 14], OperandSize::Qword)
}

fn vblendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 1445977570, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 12, 184, 226, 221, 47, 86, 115], OperandSize::Qword)
}

fn vblendps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 12, 198, 70], OperandSize::Dword)
}

fn vblendps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 12, 4, 158, 59], OperandSize::Dword)
}

fn vblendps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 12, 230, 94], OperandSize::Qword)
}

fn vblendps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1286600597, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 12, 164, 129, 149, 247, 175, 76, 57], OperandSize::Qword)
}

