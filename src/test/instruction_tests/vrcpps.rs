use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 243], OperandSize::Dword)
}

fn vrcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1214588513, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 172, 139, 97, 38, 101, 72], OperandSize::Dword)
}

fn vrcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 232], OperandSize::Qword)
}

fn vrcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1570681752, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 164, 159, 152, 179, 158, 93], OperandSize::Qword)
}

fn vrcpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 229], OperandSize::Dword)
}

fn vrcpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1091026586, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 164, 83, 154, 190, 7, 65], OperandSize::Dword)
}

fn vrcpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 225], OperandSize::Qword)
}

fn vrcpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(RDX, 1376152972, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 162, 140, 109, 6, 82], OperandSize::Qword)
}

