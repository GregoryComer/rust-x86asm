use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn mpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 224, 40], OperandSize::Dword)
}

fn mpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 60, 255, 78], OperandSize::Dword)
}

fn mpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 221, 60], OperandSize::Qword)
}

fn mpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1516786492, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 66, 156, 95, 60, 83, 104, 90, 48], OperandSize::Qword)
}

